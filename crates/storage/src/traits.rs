pub trait GetActiveModel {
    type ActiveModel: Default;

    fn get_active_model(&self) -> Self::ActiveModel {
        Self::ActiveModel::default()
    }
}
