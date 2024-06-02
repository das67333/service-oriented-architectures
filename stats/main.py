import grpc
import logging
import time
from clickhouse_driver import Client
from concurrent.futures import ThreadPoolExecutor
from stats_pb2 import *
from stats_pb2_grpc import *


MAX_WORKERS = 2
INITIAL_TIMEOUT_SEC = 1.0
POLLING_TIMEOUT_SEC = 3.0
TIMEOUT_MULTIPLIER = 1.2


def connect_db() -> Client:
    timeout = INITIAL_TIMEOUT_SEC
    while True:
        try:
            db = Client(host='stats_clickhouse')
            break
        except Exception as err:
            logging.warning(
                f'Cannot connect to Clickhouse: "{err}". Reconnecting in {timeout:.1f} seconds...')
            time.sleep(timeout)
            timeout *= TIMEOUT_MULTIPLIER
    timeout = INITIAL_TIMEOUT_SEC
    while True:
        time.sleep(timeout)
        try:
            db.execute('SELECT 1')
            return db
        except Exception as err:
            logging.warning(
                f'Cannot execute query: "{err}". Retrying in {timeout:.1f} seconds...')
            timeout *= TIMEOUT_MULTIPLIER


def init_db(db: Client):
    with open('init.sql') as f:
        commands = [s.strip() for s in f.read().split(';') if s.strip()]
    try:
        for cmd in commands:
            db.execute(cmd)
    except Exception as err:
        logging.error(f"Failed to execute command {cmd}: {err}")
        exit(1)


class Server(ServiceStatsServicer):
    def __init__(self, clickhouse_db: Client):
        super().__init__()
        self._db = clickhouse_db

    def get_post_stats(self, id: PostId, _context:  grpc.ServicerContext) -> PostStats:
        params = {'id': id.value}

        views = self._db.execute(
            'SELECT COUNT(*) FROM views_consumer WHERE post_id = %(id)s', params=params
        )[0][0]
        likes = self._db.execute(
            'SELECT COUNT(*) FROM likes_consumer WHERE post_id = %(id)s', params=params
        )[0][0]
        return PostStats(views=views, likes=likes)

    def get_top_posts(self, category: Category, _context:  grpc.ServicerContext) -> TopPosts:
        match category.value:
            case StatCategory.VIEWS:
                topic = 'views'
            case StatCategory.LIKES:
                topic = 'likes'
            case _:
                raise ValueError(f'Unknown category: {category.value}')
        result = self._db.execute('''
            SELECT post_id, login, COUNT(*) as cnt
            FROM %(table)s
            GROUP BY post_id, login
            ORDER BY cnt DESC
            LIMIT 5
        ''', params={'table': f'{topic}_stats'})
        posts = [TopPost(id=row[0], login=row[1], count=row[2])
                 for row in result]
        return TopPosts(posts=posts)

    def get_top_users(self, _empty, _context:  grpc.ServicerContext) -> TopUsers:
        result = self._db.execute('''
            SELECT login, COUNT(*) as total_likes
            FROM likes_stats
            GROUP BY login
            ORDER BY total_likes DESC
            LIMIT 3
        ''')
        users = [TopUser(login=row[0], likes=row[1])
                 for row in result]
        return TopUsers(users=users)


def main():
    logging.basicConfig(level=logging.INFO)
    server = grpc.server(ThreadPoolExecutor(max_workers=MAX_WORKERS))
    db = connect_db()
    init_db(db)
    add_ServiceStatsServicer_to_server(Server(db), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    print('hi')
    server.wait_for_termination()


if __name__ == '__main__':
    main()
