defmodule Gluesql.Store do
  alias Gluesql.Types.Option
  alias Gluesql.Types.Result

  @callback fetch_schema(storage :: any, table_name :: String.t()) :: Result.t(Option.t(Schema))
end

# async fn fetch_schema(&self, table_name: &str) -> Result<Option<Schema>>;
#
# async fn fetch_all_schemas(&self) -> Result<Vec<Schema>>;
#
# async fn fetch_data(&self, table_name: &str, key: &Key) -> Result<Option<DataRow>>;
#
# async fn scan_data(&self, table_name: &str) -> Result<RowIter>;
