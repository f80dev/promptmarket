#![no_std]
use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

//modele https://github.com/multiversx/mx-sdk-rs/blob/master/contracts/examples/crowdfunding-esdt/src/crowdfunding_esdt.rs

//On utilise des références plutôt que les données directement afin de limité la quantité de mémoire

//voir la structure https://github.com/multiversx/mx-sdk-rs/blob/master/contracts/examples/lottery-esdt/src/lottery_info.rs


#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Prompt<M: ManagedTypeApi> {
    pub text:ManagedBuffer<M>,          //contenu du prompt ou address pointant sur un prompt
    pub offer:u64,                      //Montant proposé
    pub model:u8,                        //Modele demandé
    pub inference:u16,                    //Inference
    pub scale:u16,                          //resolution
    pub owner:usize
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Render<M: ManagedTypeApi> {
    pub prompt_id:usize,
    pub url:ManagedBuffer<M>,
    pub creator:usize,
    pub price:u64                   //Prix du rendering
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait PromptMarket {
    // Storage **************************************************************************************************************************************

    //voir https://docs.multiversx.com/developers/developer-reference/storage-mappers#vecmapper-vs-linkedlistmapper
    //voir https://docs.rs/elrond-wasm/latest/elrond_wasm/storage/mappers/struct.LinkedListMapper.html

    // #[view(getMarge)]
    // #[storage_mapper("royalties")]
    // fn sc_royalties(&self) -> SingleValueMapper<u64>;        //Contient une référence d'adresse (cf_users) et l'id d'open_airdrop
    // //Représente l'historiques des bénéficiaires des airdrop ouverts pour ne pas distribuer deux fois un airdrop
    //voir exemple dans https://github.com/multiversx/mx-sdk-rs/blob/f2845bb197c3fa07e80fdf7681189d59b722a6d0/contracts/examples/rewards-distribution/src/rewards_distribution.rs#L367

    #[view(getUsers)]
    #[storage_mapper("users")]
    fn users(&self) -> UnorderedSetMapper<ManagedAddress>;


    //voir https://github.com/multiversx/mx-sdk-rs/blob/a9f22d00e3b8dca5c310a87421f75a1da064c05c/contracts/examples/nft-minter/src/nft_module.rs
    // #[view(getNftTokenId)]
    // #[storage_mapper("tokens")]
    // fn tokens(&self) -> NonFungibleTokenMapper;
    //
    #[view(getPrompts)]
    #[storage_mapper("prompts")]
    fn prompts(&self) -> VecMapper<Prompt<Self::Api>>;

    #[view(getRenders)]
    #[storage_mapper("renders")]
    fn renders(&self) -> VecMapper<Render<Self::Api>>;



    #[init]
    fn init(&self) {

    }


    fn add_user(&self,addr:ManagedAddress) -> usize {
        //Ajoute une address de user (airdropper ou claimer) sans doublon et retourne sa position
        let pos=self.users().get_index(&addr);
        if pos>0 {return pos;}
        self.users().insert(addr);
        return self.users().len();
    }



    #[endpoint]
    fn add_prompt(&self, text: ManagedBuffer,offer:u64,model:u8,inference:u16,scale:u16) {
        //Ajout d'un prompt sur le marché
        //voir https://github.com/multiversx/mx-sdk-rs/blob/a9f22d00e3b8dca5c310a87421f75a1da064c05c/contracts/examples/seed-nft-minter/src/nft_module.rs#L26
        let issue_cost = self.call_value().egld_value();
        //voir https://docs.multiversx.com/tokens/nft-tokens#issuance-of-non-fungible-tokens
        //voir https://github.com/multiversx/mx-nft-collection-minter-sc/blob/c198141d2436f41b5b3afcea24c3ebbb23d3f13b/nft-minter/src/brand_creation.rs#L148

        let prompt:Prompt<Self::Api> = Prompt {
            text:text,
            offer:offer,
            model:model,
            inference:inference,
            scale:scale,
            owner:self.add_user(self.blockchain().get_caller())
        };

        self.prompts().insert(prompt);
    }



    #[endpoint]
    #[payable("*")]
    fn get_render(&self,render_id:uszie) {
        //Récupération d'un rendu par l'acheteur

        let render:Render<Self::Api>=self.renders().get(render_id);
        let token_payment = self.call_value().egld_or_single_esdt(); //voir https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#egld_or_single_esdt

        require!(token_payment.amount>=render.price,"Le montant doit être égale au prix demandé");


        //Paiement https://docs.multiversx.com/developers/developer-reference/sc-api-functions/#direct_esdt
        direct_esdt(to: &ManagedAddress, token_id: token_payment.identifier, token_nonce: u64, amount: &BigUint)

        self.tx()
            .to(caller)
            .single_esdt(
                &tokens_mapper.get_token_id(),
                nonce,
                &BigUint::from(1u8),
            )
            .transfer();
    }


    #[endpoint]
    fn add_render(&self, prompt_id:usize,url:ManagedBuffer,price:u64) -> usize {
        /*
        Ajout d'un rendu avec son prix
        */
        let render:Render<Self::Api> = Render {
            url:url,
            prompt_id:prompt_id,
            creator:self.add_user(self.blockchain().get_caller()),
            price:price
        };
        let render_id=self.renders().insert(render);

        return render_id;
    }



}
