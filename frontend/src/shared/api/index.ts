import axios from "axios";

const BASE_URL = import.meta.env.VITE_API_URL;

const api = axios.create({
  baseURL: BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

const api_protected = axios.create({
  baseURL: BASE_URL,
  headers: {
    "Content-Type": "application/json",
    Authorization: `Bearer ${localStorage.getItem("access_token")}`,
  },
});

const api_form = axios.create({
  baseURL: BASE_URL,
  headers: {
    "Content-Type": "application/x-www-form-urlencoded",
  },
});

const api_cookies = axios.create({
  baseURL: BASE_URL,
  withCredentials: true,
  headers: {
    "Content-Type": "application/json",
  },
});

const remove_token = () => {
  localStorage.removeItem("access_token");
};

export {
  api,
  api_protected,
  api_form,
  api_cookies,
  remove_token
}
