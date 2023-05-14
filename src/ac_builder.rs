crate::generate_reqwest_client!(ActiveCampaign, {
    contact {
        search: get "contacts",
        delete: delete "contacts/{id}" id: &str,
        create: post "contacts",
        sync: post "contact/sync"
    }
});
