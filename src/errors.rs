#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error), // 例如将标准库中的 io error 统一为你定义的错误类型。以此类推，你可以将其他库中的错误统一为你声明的错误类型。
}
