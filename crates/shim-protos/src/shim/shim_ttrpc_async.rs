// This file is generated by ttrpc-compiler 0.5.1. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct TaskClient {
    client: ::ttrpc::r#async::Client,
}

impl TaskClient {
    pub fn new(client: ::ttrpc::r#async::Client) -> Self {
        TaskClient {
            client: client,
        }
    }

    pub async fn state(&self, ctx: ttrpc::context::Context, req: &super::shim::StateRequest) -> ::ttrpc::Result<super::shim::StateResponse> {
        let mut cres = super::shim::StateResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "State", cres);
    }

    pub async fn create(&self, ctx: ttrpc::context::Context, req: &super::shim::CreateTaskRequest) -> ::ttrpc::Result<super::shim::CreateTaskResponse> {
        let mut cres = super::shim::CreateTaskResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Create", cres);
    }

    pub async fn start(&self, ctx: ttrpc::context::Context, req: &super::shim::StartRequest) -> ::ttrpc::Result<super::shim::StartResponse> {
        let mut cres = super::shim::StartResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Start", cres);
    }

    pub async fn delete(&self, ctx: ttrpc::context::Context, req: &super::shim::DeleteRequest) -> ::ttrpc::Result<super::shim::DeleteResponse> {
        let mut cres = super::shim::DeleteResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Delete", cres);
    }

    pub async fn pids(&self, ctx: ttrpc::context::Context, req: &super::shim::PidsRequest) -> ::ttrpc::Result<super::shim::PidsResponse> {
        let mut cres = super::shim::PidsResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Pids", cres);
    }

    pub async fn pause(&self, ctx: ttrpc::context::Context, req: &super::shim::PauseRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Pause", cres);
    }

    pub async fn resume(&self, ctx: ttrpc::context::Context, req: &super::shim::ResumeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Resume", cres);
    }

    pub async fn checkpoint(&self, ctx: ttrpc::context::Context, req: &super::shim::CheckpointTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Checkpoint", cres);
    }

    pub async fn kill(&self, ctx: ttrpc::context::Context, req: &super::shim::KillRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Kill", cres);
    }

    pub async fn exec(&self, ctx: ttrpc::context::Context, req: &super::shim::ExecProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Exec", cres);
    }

    pub async fn resize_pty(&self, ctx: ttrpc::context::Context, req: &super::shim::ResizePtyRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "ResizePty", cres);
    }

    pub async fn close_io(&self, ctx: ttrpc::context::Context, req: &super::shim::CloseIORequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "CloseIO", cres);
    }

    pub async fn update(&self, ctx: ttrpc::context::Context, req: &super::shim::UpdateTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Update", cres);
    }

    pub async fn wait(&self, ctx: ttrpc::context::Context, req: &super::shim::WaitRequest) -> ::ttrpc::Result<super::shim::WaitResponse> {
        let mut cres = super::shim::WaitResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Wait", cres);
    }

    pub async fn stats(&self, ctx: ttrpc::context::Context, req: &super::shim::StatsRequest) -> ::ttrpc::Result<super::shim::StatsResponse> {
        let mut cres = super::shim::StatsResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Stats", cres);
    }

    pub async fn connect(&self, ctx: ttrpc::context::Context, req: &super::shim::ConnectRequest) -> ::ttrpc::Result<super::shim::ConnectResponse> {
        let mut cres = super::shim::ConnectResponse::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Connect", cres);
    }

    pub async fn shutdown(&self, ctx: ttrpc::context::Context, req: &super::shim::ShutdownRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::async_client_request!(self, ctx, req, "containerd.task.v2.Task", "Shutdown", cres);
    }
}

struct StateMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StateMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, StateRequest, state);
    }
}

struct CreateMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CreateMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, CreateTaskRequest, create);
    }
}

struct StartMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StartMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, StartRequest, start);
    }
}

struct DeleteMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for DeleteMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, DeleteRequest, delete);
    }
}

struct PidsMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for PidsMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, PidsRequest, pids);
    }
}

struct PauseMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for PauseMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, PauseRequest, pause);
    }
}

struct ResumeMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ResumeMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, ResumeRequest, resume);
    }
}

struct CheckpointMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CheckpointMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, CheckpointTaskRequest, checkpoint);
    }
}

struct KillMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for KillMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, KillRequest, kill);
    }
}

struct ExecMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ExecMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, ExecProcessRequest, exec);
    }
}

struct ResizePtyMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ResizePtyMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, ResizePtyRequest, resize_pty);
    }
}

struct CloseIoMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for CloseIoMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, CloseIORequest, close_io);
    }
}

struct UpdateMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for UpdateMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, UpdateTaskRequest, update);
    }
}

struct WaitMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for WaitMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, WaitRequest, wait);
    }
}

struct StatsMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for StatsMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, StatsRequest, stats);
    }
}

struct ConnectMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ConnectMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, ConnectRequest, connect);
    }
}

struct ShutdownMethod {
    service: Arc<std::boxed::Box<dyn Task + Send + Sync>>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for ShutdownMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<(u32, Vec<u8>)> {
        ::ttrpc::async_request_handler!(self, ctx, req, shim, ShutdownRequest, shutdown);
    }
}

#[async_trait]
pub trait Task: Sync {
    async fn state(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::StateRequest) -> ::ttrpc::Result<super::shim::StateResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/State is not supported".to_string())))
    }
    async fn create(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::CreateTaskRequest) -> ::ttrpc::Result<super::shim::CreateTaskResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Create is not supported".to_string())))
    }
    async fn start(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::StartRequest) -> ::ttrpc::Result<super::shim::StartResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Start is not supported".to_string())))
    }
    async fn delete(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::DeleteRequest) -> ::ttrpc::Result<super::shim::DeleteResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Delete is not supported".to_string())))
    }
    async fn pids(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::PidsRequest) -> ::ttrpc::Result<super::shim::PidsResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Pids is not supported".to_string())))
    }
    async fn pause(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::PauseRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Pause is not supported".to_string())))
    }
    async fn resume(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::ResumeRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Resume is not supported".to_string())))
    }
    async fn checkpoint(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::CheckpointTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Checkpoint is not supported".to_string())))
    }
    async fn kill(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::KillRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Kill is not supported".to_string())))
    }
    async fn exec(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::ExecProcessRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Exec is not supported".to_string())))
    }
    async fn resize_pty(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::ResizePtyRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/ResizePty is not supported".to_string())))
    }
    async fn close_io(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::CloseIORequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/CloseIO is not supported".to_string())))
    }
    async fn update(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::UpdateTaskRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Update is not supported".to_string())))
    }
    async fn wait(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::WaitRequest) -> ::ttrpc::Result<super::shim::WaitResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Wait is not supported".to_string())))
    }
    async fn stats(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::StatsRequest) -> ::ttrpc::Result<super::shim::StatsResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Stats is not supported".to_string())))
    }
    async fn connect(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::ConnectRequest) -> ::ttrpc::Result<super::shim::ConnectResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Connect is not supported".to_string())))
    }
    async fn shutdown(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _req: super::shim::ShutdownRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.task.v2.Task/Shutdown is not supported".to_string())))
    }
}

pub fn create_task(service: Arc<std::boxed::Box<dyn Task + Send + Sync>>) -> HashMap <String, Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/containerd.task.v2.Task/State".to_string(),
                    std::boxed::Box::new(StateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Create".to_string(),
                    std::boxed::Box::new(CreateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Start".to_string(),
                    std::boxed::Box::new(StartMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Delete".to_string(),
                    std::boxed::Box::new(DeleteMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Pids".to_string(),
                    std::boxed::Box::new(PidsMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Pause".to_string(),
                    std::boxed::Box::new(PauseMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Resume".to_string(),
                    std::boxed::Box::new(ResumeMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Checkpoint".to_string(),
                    std::boxed::Box::new(CheckpointMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Kill".to_string(),
                    std::boxed::Box::new(KillMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Exec".to_string(),
                    std::boxed::Box::new(ExecMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/ResizePty".to_string(),
                    std::boxed::Box::new(ResizePtyMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/CloseIO".to_string(),
                    std::boxed::Box::new(CloseIoMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Update".to_string(),
                    std::boxed::Box::new(UpdateMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Wait".to_string(),
                    std::boxed::Box::new(WaitMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Stats".to_string(),
                    std::boxed::Box::new(StatsMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Connect".to_string(),
                    std::boxed::Box::new(ConnectMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("/containerd.task.v2.Task/Shutdown".to_string(),
                    std::boxed::Box::new(ShutdownMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods
}
