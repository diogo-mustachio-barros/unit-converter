<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const selected_unit_type = ref("");
const selected_unit_1 = ref("");
const selected_unit_2 = ref("");
const amount_1 = ref(0);
const amount_2 = ref(0);

const unit_types = ref<string[]>([]);
const units = ref<Map<string, string[]>>(new Map<string, string[]>());

load_unit_types();

function load_unit_types() {
  invoke<string[]>("get_unit_types").then((_unit_types : string[]) => { 
    _unit_types.forEach((_unit_type:string) => { 
      unit_types.value.push(_unit_type);

      load_units(_unit_type);
    });
  });
}

function load_units(unit_type:string) {
  invoke<string[]>("get_units", {unitType: unit_type}).then((_units : string[]) => {
    units.value.set(unit_type, _units);
  });
}

function reset_selects() {
  reset_select("select_unit_1");
  reset_select("select_unit_2");
}

function reset_select(select_id:string) {
  let select : HTMLSelectElement = document.getElementById(select_id) as HTMLSelectElement;
  select.selectedIndex = 0;
}

function handle_input_change(event:Event) {
  let target = event.target as HTMLInputElement;
  let parent = target.parentElement;
  
  if (parent == null)
    return;

  if (parent.id == "form_unit_1") 
  {
    update_unit(1);
  } 
  else if (parent.id == "form_unit_2") 
  {
    update_unit(2);
  }
}

function convert(unit_type:string, amount:number, from_unit:string, to_unit:string) : Promise<number> {
  return invoke("convert", { unitType: unit_type
                    , amount: amount
                    , fromUnit: from_unit
                    , toUnit: to_unit
                    })
}

function update_unit(unit:number) {
  if (unit == 1) 
  {
    convert( selected_unit_type.value
           , amount_1.value
           , selected_unit_1.value
           , selected_unit_2.value
           )
           .then((result:number) => {
              amount_2.value = result;
            });
  } 
  else if (unit == 2) 
  {
    convert( selected_unit_type.value
           , amount_2.value
           , selected_unit_2.value
           , selected_unit_1.value
           )
           .then((result:number) => {
              amount_1.value = result;
            });
  }
}
</script>

<template>
  <div>Unit Type:</div>
  <select @change="reset_selects()" v-model="selected_unit_type">
    <option disabled value="">Please select one</option>
    <option v-for="unit in unit_types">{{ unit }}</option>
  </select>

  <div class="row">
      <div class="column">
        <div>Unit 1:</div>
        <form id="form_unit_1" v-on:submit.prevent>
          <select id="select_unit_1" @change="update_unit(2)" v-model="selected_unit_1">
            <option disabled value="">Please select one</option>
            <option v-for="unit in units.get(selected_unit_type)">{{ unit }}</option>
          </select>
          <input type="number" v-model.number="amount_1" v-on:input="handle_input_change">
        </form>
      </div>

      <div>
        <img style="width: 90%;" src="/src/assets/bidirectional-arrow.png" alt="alternatetext"> 
      </div>
      
      <div class="column">
        <div>Unit 2:</div>
        <div>
          <form id="form_unit_2" v-on:submit.prevent>
            <select id="select_unit_2" @change="update_unit(1)" v-model="selected_unit_2">
              <option disabled value="">Please select one</option>
              <option v-for="unit in units.get(selected_unit_type)">{{ unit }}</option>
            </select>
            <input type="number" v-model.number="amount_2" v-on:input="handle_input_change">
          </form>
        </div>
      </div>
  </div>
</template>
