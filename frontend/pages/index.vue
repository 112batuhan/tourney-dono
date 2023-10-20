<script setup lang="ts">
const { message, status } = useDonations();
</script>

<template>
  <p v-if="status === 'CONNECTING'" class="text-black">Loading</p>
  <div
    v-else-if="status === 'OPEN'"
    class="relative grid grid-cols-2 items-center font-bold h-24 aspect-[434/96]"
  >
    <img src="~/assets/odul_havuzu.png" class="absolute h-24 -z-10" />

    <div v-if="message?.pricepool" class="flex justify-center pt-5 pr-6 text-white">
      <p class="text-2xl">{{ message.pricepool.toLocaleString() }}tl</p>
    </div>

    <TransitionGroup
      enterActiveClass="transition-all duration-1000"
      leaveActiveClass="transition-all duration-1000 absolute"
      moveClass="transition-all duration-1000"
      leaveToClass="translate-y-full opacity-0"
      enterFromClass="translate-x-full opacity-0"
      tag="ol"
      class="text-white"
      @beforeLeave="beforeLeave"
    >
      <li
        v-for="donation in message?.aggregate_donations.slice(0, 3)"
        :key="donation.id"
      >
        <p>{{ donation.donor }} - {{ donation.amount }} TL</p>
      </li>
    </TransitionGroup>
  </div>
</template>
