export class User {
  id: number;
  name: string;
  email: string;
  is_staff: boolean;

  constructor(id: number, name: string, email: string, is_staff: boolean) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.is_staff = is_staff;
  }
}
