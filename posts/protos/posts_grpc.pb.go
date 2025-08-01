// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.21.12
// source: protos/posts.proto

package protos

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// ServicePostsClient is the client API for ServicePosts service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type ServicePostsClient interface {
	CreatePost(ctx context.Context, in *RequestCreate, opts ...grpc.CallOption) (*PostId, error)
	UpdatePost(ctx context.Context, in *RequestUpdate, opts ...grpc.CallOption) (*ReturnCode, error)
	RemovePost(ctx context.Context, in *RequestRemove, opts ...grpc.CallOption) (*ReturnCode, error)
	GetPost(ctx context.Context, in *RequestGetOne, opts ...grpc.CallOption) (*OptionalPost, error)
	GetPosts(ctx context.Context, in *RequestGetMany, opts ...grpc.CallOption) (*Posts, error)
}

type servicePostsClient struct {
	cc grpc.ClientConnInterface
}

func NewServicePostsClient(cc grpc.ClientConnInterface) ServicePostsClient {
	return &servicePostsClient{cc}
}

func (c *servicePostsClient) CreatePost(ctx context.Context, in *RequestCreate, opts ...grpc.CallOption) (*PostId, error) {
	out := new(PostId)
	err := c.cc.Invoke(ctx, "/service_posts.ServicePosts/create_post", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *servicePostsClient) UpdatePost(ctx context.Context, in *RequestUpdate, opts ...grpc.CallOption) (*ReturnCode, error) {
	out := new(ReturnCode)
	err := c.cc.Invoke(ctx, "/service_posts.ServicePosts/update_post", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *servicePostsClient) RemovePost(ctx context.Context, in *RequestRemove, opts ...grpc.CallOption) (*ReturnCode, error) {
	out := new(ReturnCode)
	err := c.cc.Invoke(ctx, "/service_posts.ServicePosts/remove_post", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *servicePostsClient) GetPost(ctx context.Context, in *RequestGetOne, opts ...grpc.CallOption) (*OptionalPost, error) {
	out := new(OptionalPost)
	err := c.cc.Invoke(ctx, "/service_posts.ServicePosts/get_post", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *servicePostsClient) GetPosts(ctx context.Context, in *RequestGetMany, opts ...grpc.CallOption) (*Posts, error) {
	out := new(Posts)
	err := c.cc.Invoke(ctx, "/service_posts.ServicePosts/get_posts", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// ServicePostsServer is the server API for ServicePosts service.
// All implementations must embed UnimplementedServicePostsServer
// for forward compatibility
type ServicePostsServer interface {
	CreatePost(context.Context, *RequestCreate) (*PostId, error)
	UpdatePost(context.Context, *RequestUpdate) (*ReturnCode, error)
	RemovePost(context.Context, *RequestRemove) (*ReturnCode, error)
	GetPost(context.Context, *RequestGetOne) (*OptionalPost, error)
	GetPosts(context.Context, *RequestGetMany) (*Posts, error)
	mustEmbedUnimplementedServicePostsServer()
}

// UnimplementedServicePostsServer must be embedded to have forward compatible implementations.
type UnimplementedServicePostsServer struct {
}

func (UnimplementedServicePostsServer) CreatePost(context.Context, *RequestCreate) (*PostId, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreatePost not implemented")
}
func (UnimplementedServicePostsServer) UpdatePost(context.Context, *RequestUpdate) (*ReturnCode, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdatePost not implemented")
}
func (UnimplementedServicePostsServer) RemovePost(context.Context, *RequestRemove) (*ReturnCode, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RemovePost not implemented")
}
func (UnimplementedServicePostsServer) GetPost(context.Context, *RequestGetOne) (*OptionalPost, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPost not implemented")
}
func (UnimplementedServicePostsServer) GetPosts(context.Context, *RequestGetMany) (*Posts, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetPosts not implemented")
}
func (UnimplementedServicePostsServer) mustEmbedUnimplementedServicePostsServer() {}

// UnsafeServicePostsServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to ServicePostsServer will
// result in compilation errors.
type UnsafeServicePostsServer interface {
	mustEmbedUnimplementedServicePostsServer()
}

func RegisterServicePostsServer(s grpc.ServiceRegistrar, srv ServicePostsServer) {
	s.RegisterService(&ServicePosts_ServiceDesc, srv)
}

func _ServicePosts_CreatePost_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RequestCreate)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ServicePostsServer).CreatePost(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/service_posts.ServicePosts/create_post",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ServicePostsServer).CreatePost(ctx, req.(*RequestCreate))
	}
	return interceptor(ctx, in, info, handler)
}

func _ServicePosts_UpdatePost_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RequestUpdate)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ServicePostsServer).UpdatePost(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/service_posts.ServicePosts/update_post",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ServicePostsServer).UpdatePost(ctx, req.(*RequestUpdate))
	}
	return interceptor(ctx, in, info, handler)
}

func _ServicePosts_RemovePost_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RequestRemove)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ServicePostsServer).RemovePost(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/service_posts.ServicePosts/remove_post",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ServicePostsServer).RemovePost(ctx, req.(*RequestRemove))
	}
	return interceptor(ctx, in, info, handler)
}

func _ServicePosts_GetPost_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RequestGetOne)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ServicePostsServer).GetPost(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/service_posts.ServicePosts/get_post",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ServicePostsServer).GetPost(ctx, req.(*RequestGetOne))
	}
	return interceptor(ctx, in, info, handler)
}

func _ServicePosts_GetPosts_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RequestGetMany)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ServicePostsServer).GetPosts(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/service_posts.ServicePosts/get_posts",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ServicePostsServer).GetPosts(ctx, req.(*RequestGetMany))
	}
	return interceptor(ctx, in, info, handler)
}

// ServicePosts_ServiceDesc is the grpc.ServiceDesc for ServicePosts service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var ServicePosts_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "service_posts.ServicePosts",
	HandlerType: (*ServicePostsServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "create_post",
			Handler:    _ServicePosts_CreatePost_Handler,
		},
		{
			MethodName: "update_post",
			Handler:    _ServicePosts_UpdatePost_Handler,
		},
		{
			MethodName: "remove_post",
			Handler:    _ServicePosts_RemovePost_Handler,
		},
		{
			MethodName: "get_post",
			Handler:    _ServicePosts_GetPost_Handler,
		},
		{
			MethodName: "get_posts",
			Handler:    _ServicePosts_GetPosts_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "protos/posts.proto",
}
