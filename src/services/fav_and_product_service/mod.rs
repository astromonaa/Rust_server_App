use crate::services::favorites_service::FavoritesService;
use crate::services::products_service::ProductsService;
use crate::db::entities::product::Model as ProductModel;
use crate::services::favorites_service::types::FavoritesErrors;

pub struct FavAndProductService {
    fav_service: FavoritesService,
    products_service: ProductsService,
}


impl FavAndProductService {
    pub fn new(fav_service: FavoritesService, products_service: ProductsService) -> FavAndProductService {
        Self {
            fav_service,
            products_service
        }
    }

    pub async fn get_favorites_list(&self, user_id: i32) -> Result<Vec<ProductModel>, FavoritesErrors> {
        let favorites = self.fav_service.get_user_favorites(user_id).await?;
        let favorites_list = self.fav_service
            .get_user_favorite_products(favorites.id)
            .await
            .map_err(|_| FavoritesErrors::GetProductError)?;

        let products_ids = favorites_list.iter().map(|p| p.product_id).collect();
        let products = self.products_service.get_products_by_ids(products_ids).await;

        Ok(products)
    }
}