import { api_protected, api_form, api_cookies } from "@/shared/api";

async function refresh_token() {
  await api_cookies.get("/users/refresh");

}

export async function load_profile(refreshed = true) {
  try {
    let profile = await api_protected.get('/users/profile').then((res) => res.data);
    return profile;
  } catch (error) {
    if (refreshed == false) {
      refresh_token();
      load_profile(refreshed=false);
    }else{
      return null;
    }
  }
}

export async function login(name: string, password: string) {
  try {
    let profile = await api_form.post('/users/login', `name=${name}&password=${password}`).then((res) => res.data);
    // localStorage.setItem("access_token", resp.access_token);
    // document.location.reload();
    return profile;
  }catch (error) {
    console.error(error);
    return null
  }
}
