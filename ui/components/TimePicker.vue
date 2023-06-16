<template>
    <h1 class="font-black text-2xl text-center mt-12">TimeTrim</h1>
    <section class="grid justify-center mt-12">
        <div class="flex gap-4 flex-shrink bg-slate-100 px-12 p-4 rounded-t-lg">
            <div class="grid grid-cols-3 gap-3">
                <label for="end" class="flex flex-col col-span-2 relative">
                    <span class="text-xs absolute top-[2px] left-[8px] text-gray-500">time</span>
                    <input class="text-2xl px-[6px] py-[4px] pt-[16px] rounded bg-slate-200" id="end" type="time" v-model="endTime">
                </label>
                <label for="break" class="flex flex-col col-span-1 relative">
                    <span class="text-xs absolute top-[2px] left-[8px] text-gray-500">break</span>
                    <input class="text-2xl px-[6px] py-[4px] pt-[16px] rounded bg-slate-200 w-[80px]" v-model="totalBreakTime" max="120" min="15" type="number" name="break" id="break">
                </label>
            </div>
        </div>
        <div class="grid grid-cols-3 gap-3 px-12 bg-green-300 rounded-b-lg py-3 relative">
            <div class="flex relative col-span-3">
                <span class="text-xs absolute top-[2px] left-[8px] text-green-700">result = time - break</span>
                <span class="text-2xl px-[6px] py-[4px] pt-[16px] rounded bg-green-200 w-full">{{ resultTime }}</span>
            </div>
        </div>
    </section>
</template>

<script setup>
import { ref, computed } from '#imports'
import { DateTime } from "luxon";

const DEFAULT_BREAK_IN_MINUTES = 45;

const currentTime = getCurrentTime();
const endTime = ref(currentTime);
const totalBreakTime = ref(DEFAULT_BREAK_IN_MINUTES);

const resultTime = computed(() => {
    let end = DateTime.fromFormat(endTime.value, "HH:mm")
    
    if (totalBreakTime.value == '') {
        return end.toFormat('HH:mm')
    }
    
    return end.minus({ minutes: totalBreakTime.value }).toFormat('HH:mm')
})

function getCurrentTime() {
    return DateTime.now().toFormat("HH:mm");
}
</script>
