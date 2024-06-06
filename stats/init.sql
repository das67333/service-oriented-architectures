CREATE TABLE IF NOT EXISTS views_kafka (post_id UInt64, login String)
ENGINE = Kafka('stats_kafka:9092', 'views', 'views_group1', 'JSONEachRow');

CREATE TABLE IF NOT EXISTS views_stats (post_id UInt64, login String)
ENGINE = MergeTree()
ORDER BY post_id;

CREATE MATERIALIZED VIEW IF NOT EXISTS views_consumer TO views_stats
AS SELECT * FROM views_kafka;


CREATE TABLE IF NOT EXISTS likes_kafka (post_id UInt64, login String)
ENGINE = Kafka('stats_kafka:9092', 'likes', 'likes_group1', 'JSONEachRow');

CREATE TABLE IF NOT EXISTS likes_stats (post_id UInt64, login String)
ENGINE = MergeTree()
ORDER BY post_id;

CREATE MATERIALIZED VIEW IF NOT EXISTS likes_consumer TO likes_stats
AS SELECT * FROM likes_kafka;