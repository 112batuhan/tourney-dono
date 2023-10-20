<script setup lang="ts">
const { celebrationDonation, celebrationDonationDonorTotal } = useDonations();
</script>

<template>
  <div class="flex flex-col items-center justify-center w-min">
    <Transition
      enterActiveClass="transition-all duration-500 delay-1000"
      leaveActiveClass="transition-all duration-500"
      enterFromClass="opacity-0 translate-y-full "
      leaveToClass="opacity-0 translate-y-full !grid-cols-[0fr] mb-0"
    >
      <div v-if="celebrationDonation" class="grid w-fit -mb-4 grid-cols-[1fr]">
        <div class="min-h-0 overflow-hidden">
          <img src="~/assets/cocuk_sadece_alkis.gif" />
        </div>
      </div>
    </Transition>
  
    <Transition
      enterActiveClass="transition-all duration-500"
      leaveActiveClass="transition-all duration-500 delay-1000"
      enterFromClass="opacity-0 scale-0"
      leaveToClass="opacity-0 scale-0"
    >
      <div
        v-if="celebrationDonation"
        class="bg-[url('~/assets/bagis_tahta.png')] z-10 grid place-content-center aspect-[333/132] h-[132px] bg-cover"
      >
        <h1>Latest Donation</h1>
        <p>
          {{ celebrationDonation.donor }} {{ celebrationDonation.amount }}
          <span v-if="celebrationDonationDonorTotal">
            - Total: {{ celebrationDonationDonorTotal }}
          </span>
        </p>
      </div>
    </Transition>
  </div>
</template>
