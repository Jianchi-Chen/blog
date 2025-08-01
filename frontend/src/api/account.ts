import type { User } from "@/types/user";
import axios from "axios";

export const registerAccount = (args: User) => {
  return axios.post("/registerAccount", args);
};
