mod trend;
mod trend_update;

pub use trend::{
    ActiveModel as TrendActiveModel, Column as TrendColumn, Entity as TrendEntity,
    Model as TrendModel, TrendMetaSource as TrendEntityMetaSource,
};

pub use trend_update::{
    ActiveModel as TrendUpdateActiveModel, Column as TrendUpdateColumn,
    Entity as TrendUpdateEntity, Model as TrendUpdateModel,
};
