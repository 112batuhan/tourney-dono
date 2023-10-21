import { Message } from "~/models/message";
import { Donation } from "~/models/donation";

export const useDonations = () => {
  const message = useState<Message | undefined>();
  const celebrationDonation = refAutoReset<Donation | undefined>(
    undefined,
    20_000
  );

  const config = useRuntimeConfig();
  const { data, status } = useWebSocket(config.public.wsUrl, {
    autoReconnect: {
      retries: 3,
    },
    heartbeat: {
      interval: 60_000,
      pongTimeout: 5000,
    },
    onMessage: () => {
      message.value = JSON.parse(data.value);

      if (!message.value?.celebration_id) return;
      const individualDonation = message.value.individual_donations.find(
        (donation) => donation.id === message.value?.celebration_id
      );

      celebrationDonation.value = individualDonation;
    },
  });

  return {
    message,
    celebrationDonation,
    status,
  };
};
