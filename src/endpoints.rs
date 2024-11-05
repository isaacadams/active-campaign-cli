api_client_macro::generate!(ActiveCampaign, {
    contact {
        #[get "contacts"]
        search(),

        #[get "contacts/{}"]
        get(id: &str),

        #[delete "contacts/{}"]
        delete(id: &str),

        #[post "contacts"]
        create(),

        #[post "contact/sync"]
        sync()
    }
});
