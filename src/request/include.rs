#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Include {
    StatisticsData,
    TagData,
    DisplayData,
    DimensionsData,
    RelationshipData,
    NeighbourData,
    VariationsData,
    DownloadData,
    PreviewData,
    MapData,
    UsdData,
    ImageData
}
impl fmt::Display for Include {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::StatisticsData => "statisticsData",
            Self::TagData => "tagData",
            Self::DisplayData => "displayData",
            Self::DimensionsData => "dimensionsData",
            Self::RelationshipData => "relationshipData",
            Self::NeighbourData => "neighbourData",
            Self::VariationsData => "variationsData",
            Self::DownloadData => "downloadData",
            Self::PreviewData => "previewData",
            Self::MapData => "mapData",
            Self::UsdData => "usdData",
            Self::ImageData => "imageData"
        };
        write!(f, "{}", repr)
    }
}