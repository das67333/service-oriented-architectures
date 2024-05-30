import unittest
from stats_pb2_grpc import ServiceStatsServicer


class TestServiceStatsServicer(unittest.TestCase):
    def test_get_post_stats(self):
        serv = ServiceStatsServicer()
        with self.assertRaises(NotImplementedError) as context:
            serv.get_post_stats(None, None)
        self.assertEqual('Method not implemented!', str(context.exception))
    
    def test_get_top_posts(self):
        serv = ServiceStatsServicer()
        with self.assertRaises(NotImplementedError) as context:
            serv.get_top_posts(None, None)
        self.assertEqual('Method not implemented!', str(context.exception))
    
    def test_get_top_users(self):
        serv = ServiceStatsServicer()
        with self.assertRaises(NotImplementedError) as context:
            serv.get_top_users(None, None)
        self.assertEqual('Method not implemented!', str(context.exception))


if __name__ == '__main__':
    unittest.main()