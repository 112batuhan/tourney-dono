<script setup lang="ts">
import zil from "~/assets/eba-zil.mp3";

const { celebrationDonation } = useDonations();

watchEffect(() => {
  if (!celebrationDonation.value) return;
  const audio = new Audio(zil);
  audio.play();
});
</script>

<template>
  <div class="flex flex-col items-center overflow-hidden w-min">
    <Transition
      enterActiveClass="transition-all duration-500 delay-1000"
      leaveActiveClass="transition-all duration-500"
      enterFromClass="opacity-0 translate-y-full h-0"
      enterToClass="h-[105px]"
      leaveFromClass="h-[105px]"
      leaveToClass="opacity-0 translate-y-full h-0"
    >
      <div v-if="celebrationDonation" class="min-h-0 overflow-hidden">
        <img src="~/assets/cocuk_sadece_alkis.gif" width="124" height="105" />
      </div>
    </Transition>

    <Transition
      enterActiveClass="transition-all duration-500"
      leaveActiveClass="transition-all duration-500 delay-500"
      enterFromClass="opacity-0 scale-0"
      leaveToClass="opacity-0 scale-0"
    >
      <div
        v-if="celebrationDonation"
        class="bg-[url('~/assets/bagis_tahta.png')] gap- text-center z-10 grid place-content-center aspect-[333/132] h-[132px] bg-cover"
      >
        <p class="font-semibold text-2xl mx-3 mt-1">
          Yeni okul katki payi ödemesi
        </p>

        <p class="font-semibold text-xl">
          {{ celebrationDonation.donor }}:
          {{ Math.round(celebrationDonation.amount) }}TL
        </p>
      </div>
    </Transition>
  </div>
</template>
