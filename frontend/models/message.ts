import { Donation } from "./donation";

export interface Message {
  aggregate_donations: Donation[];
  individual_donations: Donation[];
  celebration_id?: number;
  pricepool: number
}
