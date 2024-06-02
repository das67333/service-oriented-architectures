import unittest
from random import randint
from main import *


class MockDb:
    def __init__(self, return_values: list = []):
        self._return_values = return_values[::-1]

    def execute(self, *args, **kwargs):
        return self._return_values.pop()


class TestServerUnit(unittest.TestCase):
    def test_get_post_stats(self):
        views, likes = randint(0, 1000), randint(0, 1000)
        serv = Server(MockDb([[[views]], [[likes]]]))
        id = PostId(value=randint(0, 1000))
        stats = serv.get_post_stats(id, None)
        self.assertEqual(stats.views, views)
        self.assertEqual(stats.likes, likes)

    def test_get_top_posts(self):
        results = [[randint(0, 1000), str(randint(0, 1000)), randint(0, 1000)]
                   for _ in range(5)]
        serv = Server(MockDb([results]))
        posts = serv.get_top_posts(
            Category(value=StatCategory.VIEWS), None).posts
        for post, result in zip(posts, results, strict=True):
            self.assertEqual(post.id, result[0])
            self.assertEqual(post.login, result[1])
            self.assertEqual(post.count, result[2])

        results = [[randint(0, 1000), str(randint(0, 1000)), randint(0, 1000)]
                   for _ in range(5)]
        serv = Server(MockDb([results]))
        posts = serv.get_top_posts(
            Category(value=StatCategory.LIKES), None).posts
        for post, result in zip(posts, results, strict=True):
            self.assertEqual(post.id, result[0])
            self.assertEqual(post.login, result[1])
            self.assertEqual(post.count, result[2])

        serv = Server(MockDb())
        self.assertRaises(ValueError, serv.get_top_posts,
                          Category(value=-1), None)

    def test_get_top_users(self):
        results = [[str(randint(0, 1000)), randint(0, 1000)] for _ in range(5)]
        serv = Server(MockDb([results]))

        users = serv.get_top_users(None, None).users
        for post, result in zip(users, results, strict=True):
            self.assertEqual(post.login, result[0])
            self.assertEqual(post.likes, result[1])


if __name__ == '__main__':
    unittest.main()
