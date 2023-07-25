use derive_builder::Builder;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum EndpointType {
    Latest,
    Date(time::Date)
}

impl Default for EndpointType {
    fn default() -> Self {
        Self::Latest
    }
}

#[derive(Debug, Clone)]
pub enum Sort {
    Ascending,
    Descending
}

impl Default for Sort {
    fn default() -> Self {
        Self::Descending
    }
}

/// 
#[derive(Debug, Clone, Builder)]
pub struct EodData {
    #[builder(setter(skip))]
    endpoint: String,
    /// Specify a date in YYYY-MM-DD format. You can also specify an exact time in ISO-8601 date format, e.g. 2020-05-21T00:00:00+0000. Example: /eod/2020-01-01
    /// OR
    /// Obtain the latest available end-of-day data for one or multiple stock tickers.
    pub endpoint_type: EndpointType,
    query: EodDataQuery
}


impl Default for EodData {
    fn default() -> Self {
        Self { endpoint: String::from("/eod/"),
        endpoint_type: Default::default(),
        query: Default::default()
     }
    }
}

impl EodData {

}

#[derive(Debug, Clone, Builder, Default)]
pub struct EodDataQuery {
    ///  API access key
    pub access_key: String,
    /// Specify one or multiple stock symbols (tickers) for your request, e.g. `AAPL` or `AAPL`,`MSFT`. Each symbol consumes one API request. Maximum: 100 symbols
    pub symbols: Vec<String>,
    /// [Optional] Filter your results based on a specific stock exchange by specifying the MIC identification of a stock exchange. Example: `XNAS`
    pub exchange: Option<String>,
    /// [Optional] By default, results are sorted by date/time descending. Use this parameter to specify a sorting order. Available values: `DESC` (Default), `ASC`
    pub sort: Option<Sort>,
    /// [Optional] Filter results based on a specific timeframe by passing a from-date in YYYY-MM-DD format. You can also specify an exact time in ISO-8601 date format, e.g. 2020-05-21T00:00:00+0000
    pub date_from: Option<time::Date>,
    /// [Optional] Filter results based on a specific timeframe by passing an end-date in YYYY-MM-DD format. You can also specify an exact time in ISO-8601 date format, e.g. 2020-05-21T00:00:00+0000
    pub date_to: Option<time::Date>,
    /// [Optional] Specify a pagination limit (number of results per page) for your API request. Default limit value is 100, maximum allowed limit value is 1000.
    pub limit: Option<usize>,
    /// [Optional] Specify a pagination offset value for your API request. Example: An offset value of 100 combined with a limit value of 10 would show results 100-110. Default value is 0, starting with the first available result. 
    pub offset: Option<usize>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EodDataResponse {
    pub pagination: Pagination,
    pub data: Vec<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Returns your pagination limit value.
    pub limit: i64,
    /// Returns your pagination offset value.
    pub offset: i64,
    /// Returns the results count on the current page.
    pub count: i64,
    /// Returns the total count of results available.
    pub total: i64,
}
/// The response from the EOD data endpoint.
/// 
/// `Adjusted Prices`: "Adjusted" prices are stock price values that were amended to accurately reflect the given stock's value after accounting for any corporate actions, such as splits or dividends. Adjustments are made in accordance with the "CRSP Calculations" methodology set forth by the Center for Research in Security Prices (CRSP). 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    /// Returns the raw opening price of the given stock ticker.
    pub open: f64,
    /// Returns the raw high price of the given stock ticker.
    pub high: f64,
    /// Returns the raw low price of the given stock ticker.
    pub low: f64,
    /// Returns the raw closing price of the given stock ticker.
    pub close: f64,
    /// Returns the raw volume of the given stock ticker.
    pub volume: f64,
    /// Returns the adjusted high price of the given stock ticker.
    pub adj_high: f64,
    /// Returns the adjusted low price of the given stock ticker.
    pub adj_low: f64,
    /// Returns the adjusted closing price of the given stock ticker.
    pub adj_close: f64,
    /// Returns the adjusted opening price of the given stock ticker.
    pub adj_open: f64,
    /// Returns the adjusted volume of given stock ticker.
    pub adj_volume: f64,
    /// Returns the split factor, which is used to adjust prices when a company splits, reverse splits, or pays a distribution.
    pub split_factor: f64,
    /// Returns the dividend, which are the distribution of earnings to shareholders.
    pub dividend: f64,
    /// Returns the stock ticker symbol of the current data object.
    pub symbol: String,
    /// Returns the exchange MIC identification associated with the current data object.
    pub exchange: String,
    /// Returns the exact UTC date/time the given data was collected in ISO-8601 format.
    pub date: String,
}
