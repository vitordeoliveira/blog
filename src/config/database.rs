use firestore::FirestoreDb;
use tokio::sync::OnceCell;

use crate::config::environment;

pub async fn firestore() -> &'static FirestoreDb {
    static INSTANCE_FIRESTORE: OnceCell<FirestoreDb> = OnceCell::const_new();

    INSTANCE_FIRESTORE
        .get_or_init(|| async {
            FirestoreDb::new(environment().firestore_project_id.clone())
                .await
                .unwrap_or_else(|ex| {
                    panic!("FATAL - WHILE CREATING DATABASE INSTANCE_FIRESTORE - CAUSE: {ex}")
                })
        })
        .await
}
