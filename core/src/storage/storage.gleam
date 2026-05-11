import gleam/result
import varasto.{type TypedStorage}

import scopa.{type Model}

@external(erlang, "storage", "noop")
@external(javascript, "storage", "storage")
fn storage() -> TypedStorage(Model) {
  let assert Ok(local) = varasto.local()
  varasto.new(local, scopa.decode(), scopa.encode)
}

@external(erlang, "storage", "noop")
@external(javascript, "storage", "load")
pub fn load() -> Result(Model, Nil) {
  storage() |> varasto.get("scopa") |> result.map_error(fn(_) { Nil })
}

@external(erlang, "storage", "noop")
@external(javascript, "storage", "save")
pub fn save(model: Model) -> Result(Nil, Nil) {
  storage() |> varasto.set("scopa", model)
}
