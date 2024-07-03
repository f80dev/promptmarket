use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("mxsc:output/promptmarket.mxsc.json", promptmarket::ContractBuilder);
    return blockchain;
}
#[test]
fn init() {
    world().run("scenarios/init.scen.json");
}
#[test]
fn add_prompt() {
    world().run("scenarios/addprompt.scen.json");
}
#[test]
fn add_server() {
    world().run("scenarios/addserver.scen.json");
}

#[test]
fn add_2_prompts() {
    world().run("scenarios/add2prompts.scen.json");
}
#[test]
fn add_render() {
    world().run("scenarios/addrender.scen.json");
}
#[test]
fn get_render() {
    world().run("scenarios/getrender.scen.json");
}

#[test]
fn get_render_fail() {
    world().run("scenarios/getrender_fail.scen.json");
}

