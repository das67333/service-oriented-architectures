import unittest
from stats_pb2_grpc import ServiceStatsServicer

class DummyClass:
    def dummy_func(*args, **kwargs):
        pass

    def __getattribute__(self, name: str):
        return DummyClass.dummy_func


class TestServiceStatsServicer(unittest.TestCase):
    def test_get_post_stats(self):
        serv = ServiceStatsServicer()
        with self.assertRaises(NotImplementedError) as context:
            serv.get_post_stats(None, DummyClass())
        self.assertEqual('Method not implemented!', str(context.exception))
    
    def test_get_top_posts(self):
        serv = ServiceStatsServicer()
        with self.assertRaises(NotImplementedError) as context:
            serv.get_top_posts(None, DummyClass())
        self.assertEqual('Method not implemented!', str(context.exception))
    
    def test_get_top_users(self):
        serv = ServiceStatsServicer()
        with self.assertRaises(NotImplementedError) as context:
            serv.get_top_users(None, DummyClass())
        self.assertEqual('Method not implemented!', str(context.exception))


if __name__ == '__main__':
    unittest.main()