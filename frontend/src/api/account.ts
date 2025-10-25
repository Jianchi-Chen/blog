import type { User } from "@/types/user";
import axios from "axios";

export const registerAccount = (args: {
  username: string;
  password: string;
}) => {
  return axios.post("/api/register", args);
};

export const loginAccount = (data: { username: string; password: string }) => {
  return axios.post("/api/login", data);
};
