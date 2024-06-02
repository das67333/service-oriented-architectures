import unittest
from main import *
from random import randint
from testcontainers.clickhouse import ClickHouseContainer
from typing import Tuple


def setup_db_container() -> Tuple[ClickHouseContainer, Client]:
    container_name = 'clickhouse/clickhouse-server:24.3-alpine'
    chc = ClickHouseContainer(container_name).start()
    db = Client.from_url(chc.get_connection_url())
    init_db(db)
    return chc, db


class TestServerService(unittest.TestCase):
    def setUp(self):
        self._chc, self._db = setup_db_container()
        return super().setUp()

    def tearDown(self):
        if hasattr(self, '_chc'):
            self._chc.stop()
            self._db.disconnect()
        return super().tearDown()

    def test_get_post_stats(self):
        id, views, likes = randint(0, 1000), randint(0, 1000), randint(0, 1000)
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
        assert stats.views == views
        assert stats.likes == likes
        print(f'Post {id} stats: views={views}, likes={likes}')
        self._db.execute('TRUNCATE TABLE views_stats')
        self._db.execute('TRUNCATE TABLE likes_stats')


if __name__ == '__main__':
    unittest.main()
