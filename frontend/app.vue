<script setup lang="ts">
import { Message } from "~/models/message";
import { Donation } from "~/models/donation";

const message = ref<Message>();
const celebrationDonation = refAutoReset<Donation | undefined>(undefined, 3000);
const celebrationDonationDonorTotal = ref<number>();

const config = useRuntimeConfig();
const { data } = useWebSocket(config.public.wsUrl, {
  onMessage: () => {
    message.value = JSON.parse(data.value);

    if (!message.value?.celebration_id) return;
    const individualDonation = message.value.individual_donations.find(
      (donation) => donation.id === message.value?.celebration_id
    );

    const aggregateDonation = message.value.aggregate_donations.find(
      (donation) => donation.donor === individualDonation?.donor
    );

    celebrationDonation.value = individualDonation;
    celebrationDonationDonorTotal.value = aggregateDonation?.amount;
  },
});
</script>

<template>
  <div class="flex justify-between gap-10">
    <div>
      <h1>Latest Donation</h1>
      <Transition
        enterActiveClass="transition-all duration-500"
        leaveActiveClass="transition-all duration-500"
        enterFromClass="opacity-0 -translate-x-full"
        leaveToClass="opacity-0 -translate-x-full"
      >
        <p v-if="celebrationDonation">
          {{ celebrationDonation.donor }} {{ celebrationDonation.amount }}
          <span v-if="celebrationDonationDonorTotal">
            - Total: {{celebrationDonationDonorTotal }}
          </span>
        </p>
      </Transition>
    </div>

    <div>
      <h1>Aggregate Donations</h1>
      <TransitionGroup
        enterActiveClass="transition-all duration-500"
        leaveActiveClass="transition-all duration-500"
        moveClass="transition-all duration-500"
        leaveToClass="absolute translate-y-full opacity-0"
        enterFromClass="translate-x-full opacity-0"
        tag="ol"
      >
        <li
          v-for="donation in message?.aggregate_donations.slice(0, 3)"
          :key="donation.id"
        >
          <p>{{ donation.donor }} - {{ donation.amount }}₺</p>
        </li>
      </TransitionGroup>
    </div>
  </div>
</template>
