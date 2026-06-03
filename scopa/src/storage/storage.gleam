import gleam/bit_array
import gleam/dynamic/decode.{type Decoder}
import gleam/json.{type Json}
import gleam/result
import varasto.{type TypedStorage}

@external(erlang, "storage", "noop")
fn storage(decoder: Decoder(a), encoder: fn(a) -> Json) -> TypedStorage(a) {
  let assert Ok(local) = varasto.local()

  let obfuscated_reader =
    decode.string
    |> decode.then(fn(b64_str) {
      let parsed_result = {
        use bits <- result.try(bit_array.base64_decode(b64_str))
        use json_str <- result.try(bit_array.to_string(bits))
        json.parse(from: json_str, using: decoder)
        |> result.replace_error(Nil)
      }

      case parsed_result {
        Ok(data) -> decode.success(data)
        Error(_) -> decoder
      }
    })

  let obfuscated_writer = fn(value: a) -> Json {
    encoder(value)
    |> json.to_string
    |> bit_array.from_string
    |> bit_array.base64_encode(True)
    |> json.string
  }

  varasto.new(local, obfuscated_reader, obfuscated_writer)
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
}
