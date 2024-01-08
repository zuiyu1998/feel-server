mod lable;

mod user_lable;

pub use lable::{
    ActiveModel as LabelActiveModel, Column as LabelColumn, Entity as LabelEntity,
    Model as LabelModel,
};

pub use user_lable::{
    ActiveModel as UserLabelActiveModel, Column as UserLabelColumn, Entity as UserLabelEntity,
    Model as UserLabelModel,
};
