<script setup lang="ts">
const { message, status } = useDonations();

const filteredDonations = computed(() =>
  message.value?.aggregate_donations.filter(
    (donation) => donation.donor !== "heyronii"
  )
);

const lastDonation = computed(
  () =>
    message.value?.individual_donations.filter(
      (donation) => donation.donor !== "heyronii"
    )[0]
);
</script>

<template>
  <p v-if="status === 'CONNECTING'" class="text-black">Loading</p>
  <div
    v-else-if="status === 'OPEN'"
    class="relative grid grid-cols-2 h-24 aspect-[434/96]"
  >
    <img src="~/assets/odul_havuzu.png" class="absolute h-24 -z-10" />
    <div
      v-if="message?.pricepool"
      class="flex flex-col justify-start text-center mt-4"
    >
      <p class="text-3xl text-purple-300">Ödül Havuzu</p>
      <p class="text-2xl -mt-1 text-yellow-500">
        {{ message.pricepool.toLocaleString().replace(",", "‚") }}TL
      </p>
    </div>

    <div class="my-2 overflow-hidden">
      <div class="scroll">
        <div class="text-center">
          <div class="text-2xl my-1">Son Bagis</div>
          <div class="mb-20 text-xl">
            {{ lastDonation?.donor }} - {{ lastDonation?.amount }}TL
          </div>
        </div>

        <div v-for="(donation, index) in filteredDonations" :key="donation.id">
          <span>{{ index + 1 }}-</span>
          {{ donation.donor }} {{ Math.round(donation.amount) }}TL
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.scroll {
  margin-top: 0.6em;
  animation-name: scroll;
  animation-duration: 30s;
  animation-iteration-count: infinite;
  animation-timing-function: ease-out;
}

@keyframes scroll {
  0% {
    transform: translateY(-100%);
  }
  50% {
    transform: translateY(-9.4em);
  }
  70% {
    transform: translateY(-9.4em);
  }
  80% {
    transform: translateY(0);
  }
  95% {
    transform: translateY(0);
  }
  100% {
    transform: translateY(4.5em);
  }
}
</style>
