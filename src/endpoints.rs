crate::generate_reqwest_client!(ActiveCampaign, {
    contact {
        search: get "contacts",
        get: get "contacts/{id}" id: &str,
        delete: delete "contacts/{id}" id: &str,
        create: post "contacts",
        sync: post "contact/sync"
    }
});
