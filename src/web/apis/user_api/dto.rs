use poem_openapi::Union;
use poem_openapi::{types::Any, Enum, Object};

use rc_storage::prelude::{
    Article, ArticleForm, AuthClass, Label, MetaDetail, Trend, TrendDetail, TrendForm, TrendMeta,
    TrendMetaSource, TrendParams, User, UserDetail, UserForm, UserLabel, UserLoginForm,
};

use rc_storage::chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct ArticleFormRequest {
    pub title: String,
    pub cover: String,
    pub content: String,
}

impl ArticleFormRequest {
    pub fn get_form(&self, user_id: i32) -> ArticleForm {
        ArticleForm {
            user_id,
            title: self.title.clone(),
            cover: self.cover.clone(),
            content: self.content.clone(),
        }
    }
}

#[derive(Debug, Object)]
pub struct ArticleResponse {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub cover: String,
    pub content: String,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl ArticleResponse {
    pub fn from_article(article: Article) -> ArticleResponse {
        ArticleResponse {
            id: article.id,
            user_id: article.user_id,
            content: article.content,
            like_count: article.like_count,
            unlike_count: article.unlike_count,
            create_at: Any(article.create_at),
            update_at: Any(article.update_at),
            title: article.title,
            cover: article.cover,
        }
    }
}

#[derive(Enum, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum AuthClassRequest {
    Email,
}

impl From<AuthClassRequest> for AuthClass {
    fn from(value: AuthClassRequest) -> Self {
        match value {
            AuthClassRequest::Email => AuthClass::Email,
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct AddLabelFormRequest {
    pub lable_id: i32,
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct TrendParamsRequest {
    pub page: u64,
    pub page_size: u64,
}

impl TrendParamsRequest {
    pub fn get_params(&self, user_id: i32) -> TrendParams {
        TrendParams {
            page: self.page,
            page_size: self.page_size,
            user_id: Some(user_id),
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct TrendFormRequest {
    pub content: String,
}

impl TrendFormRequest {
    pub fn get_form(&self, user_id: i32) -> TrendForm {
        TrendForm {
            user_id,
            content: self.content.clone(),
            meta: None,
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct UserLoginFormRequest {
    pub auth_class: AuthClassRequest,
    pub username: String,
    pub password: String,
}

impl UserLoginFormRequest {
    pub fn get_user_form(&self) -> UserLoginForm {
        UserLoginForm {
            auth_class: AuthClass::from(self.auth_class),
            auth_name: self.username.clone(),
            auth_data: self.password.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Object, Serialize)]
pub struct UserFormRequest {
    pub avatar: String,
    pub auth_class: AuthClassRequest,
    pub auth_name: String,
    pub auth_data: String,
}

impl UserFormRequest {
    pub fn get_user_form(&self) -> UserForm {
        UserForm {
            avatar: self.avatar.clone(),
            auth_class: AuthClass::from(self.auth_class),
            auth_name: self.auth_name.clone(),
            auth_data: self.auth_data.clone(),
        }
    }
}

#[derive(Debug, Object)]
pub struct UserLabelResponse {
    pub id: i32,
    pub user_id: i32,
    pub label_id: i32,
    pub sequence: i32,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
}

impl UserLabelResponse {
    pub fn from_user_label(user_label: UserLabel) -> UserLabelResponse {
        UserLabelResponse {
            id: user_label.id,
            user_id: user_label.user_id,
            label_id: user_label.label_id,
            sequence: user_label.sequence,
            create_at: Any(user_label.create_at),
            update_at: Any(user_label.update_at),
        }
    }
}

#[derive(Debug, Object)]
pub struct LabelResponse {
    pub id: i32,
    pub user_id: i32,
    pub sequence: i32,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub description: String,
    pub name: String,
    pub effect: i64,
}

impl LabelResponse {
    pub fn from_label(label: Label) -> LabelResponse {
        LabelResponse {
            id: label.id,
            user_id: label.user_id,
            sequence: label.sequence,
            description: label.description,
            name: label.name,
            effect: label.effect,
            create_at: Any(label.create_at),
            update_at: Any(label.update_at),
        }
    }
}

#[derive(Enum, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum TrendMetaSourceEnum {
    Article,
}

impl From<TrendMetaSource> for TrendMetaSourceEnum {
    fn from(value: TrendMetaSource) -> Self {
        match value {
            TrendMetaSource::Article => TrendMetaSourceEnum::Article,
        }
    }
}

#[derive(Debug, Object)]
pub struct TrendMetaResponse {
    pub source: TrendMetaSourceEnum,
    pub source_id: i32,
}

impl TrendMetaResponse {
    pub fn from_meta(trend_meta: TrendMeta) -> TrendMetaResponse {
        TrendMetaResponse {
            source: trend_meta.source.into(),
            source_id: trend_meta.source_id,
        }
    }
}

#[derive(Debug, Object)]
pub struct TrendListResponse {
    pub data: Vec<TrendDetailResponse>,
    pub has_next: bool,
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, Object)]
pub struct TrendResponse {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub meta: Option<TrendMetaResponse>,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub like_count: i32,
    pub unlike_count: i32,
}

impl TrendResponse {
    pub fn from_trend(trend: Trend) -> TrendResponse {
        TrendResponse {
            id: trend.id,
            user_id: trend.user_id,
            content: trend.content,
            like_count: trend.like_count,
            unlike_count: trend.unlike_count,
            create_at: Any(trend.create_at),
            update_at: Any(trend.update_at),
            meta: trend
                .meta
                .and_then(|meta| Some(TrendMetaResponse::from_meta(meta))),
        }
    }
}

#[derive(Debug, Object)]
pub struct TrendDetailResponse {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub meta: Option<TrendMetaResponse>,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub like_count: i32,
    pub unlike_count: i32,
    pub meta_value: Option<MetaDetailResponse>,
    pub user: Option<UserResponse>,
}

#[derive(Union, Debug)]
#[oai(discriminator_name = "meta_source")]
pub enum MetaDetailResponse {
    Article(ArticleResponse),
}

impl MetaDetailResponse {
    pub fn from_meta_detail(meta_detail: MetaDetail) -> MetaDetailResponse {
        match meta_detail {
            MetaDetail::Article(artilce) => {
                MetaDetailResponse::Article(ArticleResponse::from_article(artilce))
            }
        }
    }
}

impl TrendDetailResponse {
    pub fn from_trend(trend: TrendDetail) -> TrendDetailResponse {
        TrendDetailResponse {
            user: trend
                .user
                .and_then(|user| Some(UserResponse::from_user(user))),
            meta_value: trend
                .meta_detail
                .and_then(|meta_detail| Some(MetaDetailResponse::from_meta_detail(meta_detail))),
            id: trend.id,
            user_id: trend.user_id,
            content: trend.content,
            like_count: trend.like_count,
            unlike_count: trend.unlike_count,
            create_at: Any(trend.create_at),
            update_at: Any(trend.update_at),
            meta: trend
                .meta
                .and_then(|meta| Some(TrendMetaResponse::from_meta(meta))),
        }
    }
}

#[derive(Debug, Object)]
pub struct UserResponse {
    pub id: i32,
    pub nikename: String,
    pub uid: String,
    pub avatar: String,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
}

impl UserResponse {
    pub fn from_user(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            nikename: user.nikename,
            uid: user.uid,
            avatar: user.avatar,
            create_at: Any(user.create_at),
            update_at: Any(user.update_at),
        }
    }
}

#[derive(Debug, Object)]
pub struct UserDetailResponse {
    pub id: i32,
    pub nikename: String,
    pub uid: String,
    pub avatar: String,
    pub create_at: Any<NaiveDateTime>,
    pub update_at: Any<NaiveDateTime>,
    pub like_count: i32,
    pub follow_count: i32,
}

impl UserDetailResponse {
    pub fn from_user(user: UserDetail) -> UserDetailResponse {
        UserDetailResponse {
            id: user.id,
            nikename: user.nikename,
            uid: user.uid,
            avatar: user.avatar,
            create_at: Any(user.create_at),
            update_at: Any(user.update_at),
            like_count: user.like_count,
            follow_count: user.follow_count,
        }
    }
}
