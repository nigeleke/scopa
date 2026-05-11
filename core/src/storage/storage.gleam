import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/result
import varasto.{type TypedStorage}

@external(erlang, "storage", "noop")
fn storage(decoder: Decoder(a), encoder: fn(a) -> Json) -> TypedStorage(a) {
  let assert Ok(local) = varasto.local()
  varasto.new(local, decoder, encoder)
}

@external(erlang, "storage", "noop")
pub fn load(
  key: String,
  decoder: Decoder(a),
  encoder: fn(a) -> Json,
) -> Result(a, Nil) {
  let storage = storage(decoder, encoder)

  storage
  |> varasto.get(key)
  |> result.replace_error(Nil)
}

@external(erlang, "storage", "noop")
pub fn save(
  key: String,
  value: a,
  decoder: Decoder(a),
  encoder: fn(a) -> Json,
) -> Result(Nil, Nil) {
  let storage = storage(decoder, encoder)

  storage
  |> varasto.set(key, value)
  |> result.replace_error(Nil)
}
