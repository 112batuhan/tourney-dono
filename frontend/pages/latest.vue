<script setup lang="ts">
const { celebrationDonation, celebrationDonationDonorTotal } = useDonations();
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
        class="bg-[url('~/assets/bagis_tahta.png')] gap-2 text-center z-10 grid place-content-center aspect-[333/132] h-[132px] bg-cover"
      >
        <p class="text-xl font-semibold">Yeni okul katkı payı ödemesi</p>

        <div>
          <p class="text-lg font-semibold">
            {{ celebrationDonation.donor }} {{ celebrationDonation.amount }} TL
          </p>
          <p v-if="celebrationDonationDonorTotal" class="text-sm">
            Toplam: {{ celebrationDonationDonorTotal }} TL
          </p>
        </div>

        <!-- <p>
          {{ celebrationDonation.donor }} {{ celebrationDonation.amount }} TL
          <span v-if="celebrationDonationDonorTotal">
            Toplam: {{ celebrationDonationDonorTotal }} TL
          </span>
        </p> -->
      </div>
    </Transition>
  </div>
</template>
