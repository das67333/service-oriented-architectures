from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class StatCategory(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = ()
    VIEWS: _ClassVar[StatCategory]
    LIKES: _ClassVar[StatCategory]
VIEWS: StatCategory
LIKES: StatCategory

class PostId(_message.Message):
    __slots__ = ("value",)
    VALUE_FIELD_NUMBER: _ClassVar[int]
    value: int
    def __init__(self, value: _Optional[int] = ...) -> None: ...

class Category(_message.Message):
    __slots__ = ("value",)
    VALUE_FIELD_NUMBER: _ClassVar[int]
    value: StatCategory
    def __init__(self, value: _Optional[_Union[StatCategory, str]] = ...) -> None: ...

class PostStats(_message.Message):
    __slots__ = ("views", "likes")
    VIEWS_FIELD_NUMBER: _ClassVar[int]
    LIKES_FIELD_NUMBER: _ClassVar[int]
    views: int
    likes: int
    def __init__(self, views: _Optional[int] = ..., likes: _Optional[int] = ...) -> None: ...

class TopPost(_message.Message):
    __slots__ = ("id", "login", "count")
    ID_FIELD_NUMBER: _ClassVar[int]
    LOGIN_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    id: int
    login: str
    count: int
    def __init__(self, id: _Optional[int] = ..., login: _Optional[str] = ..., count: _Optional[int] = ...) -> None: ...

class TopPosts(_message.Message):
    __slots__ = ("posts",)
    POSTS_FIELD_NUMBER: _ClassVar[int]
    posts: _containers.RepeatedCompositeFieldContainer[TopPost]
    def __init__(self, posts: _Optional[_Iterable[_Union[TopPost, _Mapping]]] = ...) -> None: ...

class TopUser(_message.Message):
    __slots__ = ("login", "likes")
    LOGIN_FIELD_NUMBER: _ClassVar[int]
    LIKES_FIELD_NUMBER: _ClassVar[int]
    login: str
    likes: int
    def __init__(self, login: _Optional[str] = ..., likes: _Optional[int] = ...) -> None: ...

class TopUsers(_message.Message):
    __slots__ = ("users",)
    USERS_FIELD_NUMBER: _ClassVar[int]
    users: _containers.RepeatedCompositeFieldContainer[TopUser]
    def __init__(self, users: _Optional[_Iterable[_Union[TopUser, _Mapping]]] = ...) -> None: ...
