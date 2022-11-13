

// #[derive(PartialEq, Eq, Store, Default)]
// pub struct DataState {
//     pub data: DataDictionary,
// }

// impl DataState {
//     pub async fn setup() {
//         let result = Self::create().await.unwrap();
//         Dispatch::<Self>::new().set(result);
//     }

//     pub async fn create() -> Result<Self, anyhow::Error> {
//         let url = "https://docs.google.com/spreadsheets/d/e/2PACX-1vS6n_bJgC33CE8b9o51niMx5g8WDr56x9-XIY_i61C5Bw0Cbs0vjEo0M4zIXPdX-mp8ooWx0fOICuMZ/pub?gid=0&single=true&output=tsv";
//         let result = reqwest::get(url).await;
//         let response = result?;
//         //let text = data.text().await?;

//         let bytes = response.bytes().await?;
//         //let reader = bytes.reader();

//         let mut rdr: Reader<_> = csv::Reader::from_reader(bytes.reader());

//         let rows: Vec<DataRow> = rdr.deserialize().try_collect()?;

//         let data: DataDictionary = rows.try_into()?;

//         Ok(Self { data })
//     }
// }
