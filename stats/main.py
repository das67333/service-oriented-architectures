from fastapi import FastAPI
import asyncio
import clickhouse_connect
import logging

INITIAL_TIMEOUT_SEC = 1.0
POLLING_TIMEOUT_SEC = 3.0
TIMEOUT_MULTIPLIER = 1.2

logging.basicConfig(level=logging.INFO)
app = FastAPI(debug=True)


async def init_clickhouse():
    timeout = INITIAL_TIMEOUT_SEC
    while True:
        try:
            ch = clickhouse_connect.get_client(host='stats_clickhouse')
            break
        except Exception as err:
            logging.warning(f'Cannot connect to Clickhouse: "{err}". Reconnecting in {timeout:.1f} seconds...')
            await asyncio.sleep(timeout)
            timeout *= TIMEOUT_MULTIPLIER

    # creating tables
    for topic in ('views', 'likes'):
        try:
            ch.command(f'''
                CREATE TABLE IF NOT EXISTS {topic}_kafka (post_id UInt64, login String)
                ENGINE = Kafka('stats_kafka:9092', '{topic}', '{topic}_group1', 'JSONEachRow');
            ''')
            ch.command(f'''
                CREATE TABLE IF NOT EXISTS {topic}_stats (post_id UInt64, login String)
                ENGINE = MergeTree()
                ORDER BY post_id;
            ''')
            ch.command(f'''
                CREATE MATERIALIZED VIEW IF NOT EXISTS {topic}_consumer TO {topic}_stats
                AS SELECT * FROM {topic}_kafka;
            ''')
        except Exception as err:
            logging.error(f"Failed to execute commands for topic {topic}: {err}")
            exit(1)
    return ch


async def poll_clickhouse(ch):
    while True:
        for table in ('views_consumer', 'likes_consumer'):
            try:
                result = ch.query(f'SELECT * FROM {table} ORDER BY post_id, login')
                logging.info(f'Table "{table}": {result.result_rows}')
                await asyncio.sleep(POLLING_TIMEOUT_SEC)
            except Exception as err:
                logging.error(f"Failed to query table {table}: {err}")
                exit(1)

async def main():
    ch = await init_clickhouse()
    await poll_clickhouse(ch)


@app.get("/")
def always_ok():
    return 'OK'

asyncio.create_task(main())
