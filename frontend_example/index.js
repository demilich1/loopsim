var state = {}
state.loop_no = 1;
state.diff_scale = 0.95;
state.hero = {};
state.hero.max_hp = 250;
state.hero.defense = 0;
state.hero.dmg_min = 4;
state.hero.dmg_max = 6;
state.hero.dmg_all = 0;
state.hero.magic_dmg = 0;
state.hero.evade = 0;
state.hero.counter = 0;
state.hero.vampirism = 0;
state.hero.atk_spd = 0;
state.hero.regen = 0.6;

window.onload = () => updateUiState(state)

function execute() {
    var loopValue = document.getElementById("inputLoop").value;
    var diffScaleValue = document.getElementById("selectDifficulty").value;
    var maxHpValue = document.getElementById("inputMaxHp").value;
    var defenseValue = document.getElementById("inputDefense").value;
    var dmgMinValue = document.getElementById("inputDmgMin").value;
    var dmgMaxValue = document.getElementById("inputDmgMax").value;
    var dmgAllValue = document.getElementById("inputDmgAll").value;
    var dmgMagicValue = document.getElementById("inputDmgMagic").value;
    var evadeValue = document.getElementById("inputEvade").value;
    var counterValue = document.getElementById("inputCounter").value;
    var attackSpeedValue = document.getElementById("inputAttackSpeed").value;
    var vampirismValue = document.getElementById("inputVampirism").value;
    var regenValue = document.getElementById("inputRegen").value;

    //TODO: validation
    state.loop_no = loopValue;
    state.diff_scale = diffScaleValue;
    state.hero = {};
    state.hero.max_hp = maxHpValue;
    state.hero.defense = defenseValue;
    state.hero.dmg_min = dmgMinValue
    state.hero.dmg_max = dmgMaxValue
    state.hero.dmg_all = dmgAllValue
    state.hero.magic_dmg = dmgMagicValue
    state.hero.evade = evadeValue;
    state.hero.counter = counterValue;
    state.hero.vampirism = vampirismValue;
    state.hero.atk_spd = attackSpeedValue;
    state.hero.regen = regenValue;

    alert(loopValue)
}

function updateUiState(newState) {
    document.getElementById("inputLoop").value = newState.loop_no;
    document.getElementById("selectDifficulty").value = newState.diff_scale;
    document.getElementById("inputMaxHp").value = newState.hero.max_hp;
    document.getElementById("inputDefense").value = newState.hero.defense;
    document.getElementById("inputDmgMin").value = newState.hero.dmg_min;
    document.getElementById("inputDmgMax").value = newState.hero.dmg_max;
    document.getElementById("inputDmgAll").value = newState.hero.dmg_all;
    document.getElementById("inputDmgMagic").value = newState.hero.magic_dmg;
    document.getElementById("inputEvade").value = newState.hero.evade;
    document.getElementById("inputCounter").value = newState.hero.counter;
    document.getElementById("inputAttackSpeed").value = newState.hero.atk_spd;
    document.getElementById("inputVampirism").value = newState.hero.vampirism;
    document.getElementById("inputRegen").value = newState.hero.regen;
}