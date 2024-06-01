import clickhouse_connect
import grpc
import logging
import time
from concurrent.futures import ThreadPoolExecutor
from stats_pb2 import *
from stats_pb2_grpc import *


MAX_WORKERS = 2
INITIAL_TIMEOUT_SEC = 1.0
POLLING_TIMEOUT_SEC = 3.0
TIMEOUT_MULTIPLIER = 1.2


def init_clickhouse():
    timeout = INITIAL_TIMEOUT_SEC
    while True:
        try:
            ch = clickhouse_connect.get_client(host='stats_clickhouse')
            break
        except Exception as err:
            logging.warning(
                f'Cannot connect to Clickhouse: "{err}". Reconnecting in {timeout:.1f} seconds...')
            time.sleep(timeout)
            timeout *= TIMEOUT_MULTIPLIER

    # creating tables
    with open('init.sql') as f:
        try:
            for cmd in f.read().split(';'):
                cmd = cmd.strip()
                if cmd:
                    ch.command(cmd)
        except Exception as err:
            logging.error(f"Failed to execute command {cmd}: {err}")
            exit(1)
    return ch


class Server(ServiceStatsServicer):
    def __init__(self, clickhouse_db):
        super().__init__()
        self._ch = clickhouse_db

    def get_post_stats(self, id: PostId, _context:  grpc.ServicerContext) -> PostStats:
        params = {'id': id.value}

        views = self._ch.command(
            'SELECT COUNT(*) FROM views_consumer WHERE post_id = {id:UInt64}', parameters=params)
        likes = self._ch.command(
            'SELECT COUNT(*) FROM likes_consumer WHERE post_id = {id:UInt64}', parameters=params)
        return PostStats(views=views, likes=likes)

    def get_top_posts(self, category: Category, _context:  grpc.ServicerContext) -> TopPosts:
        match category.value:
            case StatCategory.VIEWS:
                topic = 'views'
            case StatCategory.LIKES:
                topic = 'likes'
            case _:
                raise ValueError(f'Unknown category: {category.value}')
        result = self._ch.query(f'''
            SELECT post_id, login, COUNT(*) as cnt
            FROM {topic}_stats
            GROUP BY post_id, login
            ORDER BY cnt DESC
            LIMIT 5
        ''')
        posts = [TopPost(id=row[0], login=row[1], count=row[2])
                 for row in result.result_rows]
        return TopPosts(posts=posts)

    def get_top_users(self, _empty, _context:  grpc.ServicerContext) -> TopUsers:
        result = self._ch.query(f'''
            SELECT login, COUNT(*) as total_likes
            FROM likes_stats
            GROUP BY login
            ORDER BY total_likes DESC
            LIMIT 3
        ''')
        users = [TopUser(login=row[0], likes=row[1])
                 for row in result.result_rows]
        return TopUsers(users=users)


def main():
    logging.basicConfig(level=logging.INFO)
    server = grpc.server(ThreadPoolExecutor(max_workers=MAX_WORKERS))
    ch = init_clickhouse()
    add_ServiceStatsServicer_to_server(Server(ch), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    server.wait_for_termination()


if __name__ == '__main__':
    main()
