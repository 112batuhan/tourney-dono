import { Message } from "~/models/message";
import { Donation } from "~/models/donation";

export const useDonations = () => {
  const message = useState<Message>();
  const celebrationDonation = refAutoReset<Donation | undefined>(
    undefined,
    7000
  );
  const celebrationDonationDonorTotal = useState<number | undefined>();

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

  return {
    message,
    celebrationDonation,
    celebrationDonationDonorTotal
  }
};
