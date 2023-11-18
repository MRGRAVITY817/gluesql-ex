defmodule Gluesql.Types.Result do
  @type t(a) :: {:error, String.t()} | {:ok, a}
end
