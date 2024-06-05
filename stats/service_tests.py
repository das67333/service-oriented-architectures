import unittest
from main import *
from random import randint
from testcontainers.core.network import Network
from testcontainers.clickhouse import ClickHouseContainer
from testcontainers.kafka import KafkaContainer, dedent
from confluent_kafka import Producer

KAFKA_FLUSH_TIMEOUT = 10.
KAFKA_IMAGE = 'confluentinc/cp-kafka:7.6.1'
CLICKHOUSE_IMAGE = 'clickhouse/clickhouse-server:24.3-alpine'


# Fix for KafkaContainer to run in user-defined network
class KafkaContainerFixed(KafkaContainer):
    def tc_start(self) -> None:
        host = self.get_container_host_ip()
        port = self.get_exposed_port(self.port)
        # When Kafka is connected to a user-defined network (testcontainers.core.network.Network),
        # the command `hostname -i` returns multiple IPs (e.g., "172.17.0.3 192.168.16.2").
        # This can cause the Kafka broker to fail to start. To resolve this, we select the lexicographically
        # largest IP address, which is typically associated with the user-defined network.
        #
        # INSTEAD OF:
        # listeners = f"PLAINTEXT://{host}:{port},BROKER://$(hostname -i):9092"
        listeners = f"PLAINTEXT://{host}:{port},BROKER://$(hostname -i | tr ' ' '\n' | sort | tail -n 1):9092"
        data = (
            dedent(
                f"""
                #!/bin/bash
                echo 'clientPort=2181' > zookeeper.properties
                echo 'dataDir=/var/lib/zookeeper/data' >> zookeeper.properties
                echo 'dataLogDir=/var/lib/zookeeper/log' >> zookeeper.properties
                zookeeper-server-start zookeeper.properties &
                export KAFKA_ZOOKEEPER_CONNECT='localhost:2181'
                export KAFKA_ADVERTISED_LISTENERS={listeners}
                . /etc/confluent/docker/bash-config
                /etc/confluent/docker/configure
                /etc/confluent/docker/launch
                """
            )
            .strip()
            .encode("utf-8")
        )
        self.create_file(data, KafkaContainer.TC_START_SCRIPT)


class ServiceTestsClickhouse(unittest.TestCase):
    def setUp(self):
        self._clickhouse_container = ClickHouseContainer(
            CLICKHOUSE_IMAGE).start()
        url = self._clickhouse_container.get_connection_url()
        self._db = Client.from_url(url)
        init_db(self._db)
        return super().setUp()

    def tearDown(self):
        self._db.disconnect()
        self._clickhouse_container.stop()
        return super().tearDown()

    def test_get_post_stats(self):
        id, views, likes = randint(0, 1000), randint(1, 1000), randint(1, 1000)
        login = str(randint(100, 999))
        self._db.execute(
            'INSERT INTO views_stats (post_id, login) VALUES',
            [(id, login) for _ in range(views)]
        )
        self._db.execute(
            'INSERT INTO likes_stats (post_id, login) VALUES',
            [(id, login) for _ in range(likes)]
        )

        serv = Server(self._db)
        stats = serv.get_post_stats(PostId(value=id), None)
        assert stats.views == views, f'{stats.views} != {views}'
        assert stats.likes == likes, f'{stats.likes} != {likes}'
        self._db.execute('TRUNCATE TABLE views_stats')
        self._db.execute('TRUNCATE TABLE likes_stats')


class ServiceTestsKafka(unittest.TestCase):
    def setUp(self):
        self._network = Network().__enter__()

        self._kafka_container = KafkaContainerFixed(KAFKA_IMAGE).with_name(
            'stats_kafka').with_network(self._network).start()
        server = self._kafka_container.get_bootstrap_server()
        self._kafka_producer = Producer({'bootstrap.servers': server})

        self._clickhouse_container = ClickHouseContainer(
            CLICKHOUSE_IMAGE).with_network(self._network).start()
        url = self._clickhouse_container.get_connection_url()
        self._db = Client.from_url(url)

        init_db(self._db)
        return super().setUp()

    def tearDown(self):
        self._db.disconnect()
        self._clickhouse_container.stop()
        self._kafka_producer.purge()
        self._kafka_container.stop()
        self._network.remove()
        return super().tearDown()

    def test_get_post_stats(self):
        id, views, likes = randint(0, 1000), randint(1, 1000), randint(1, 1000)
        login = str(randint(100, 999))
        msg = f'{{ "post_id": {id}, "login": "{login}" }}'
        for _ in range(views):
            self._kafka_producer.produce('views', msg.encode())
        for _ in range(likes):
            self._kafka_producer.produce('likes', msg.encode())
        self._kafka_producer.flush()
        time.sleep(KAFKA_FLUSH_TIMEOUT)

        serv = Server(self._db)
        stats = serv.get_post_stats(PostId(value=id), None)
        assert stats.views == views, f'{stats.views} != {views}'
        assert stats.likes == likes, f'{stats.likes} != {likes}'
        self._db.execute('TRUNCATE TABLE views_stats')
        self._db.execute('TRUNCATE TABLE likes_stats')


if __name__ == '__main__':
    unittest.main()
