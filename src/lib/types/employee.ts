export enum EmployeeStatus {
  APPLIED = "applied",
  CURRENT = "current",
  PAST = "past",
}

export interface Employee {
  id: number;
  name: string;
  fatherName?: string;
  spouseName?: string;
  currentPlace?: string;
  currentPost?: string;
  currentAddress?: string;
  phoneNumbers?: string;
  permanentSameAsCurrent: number;
  permanentPlace?: string;
  permanentPost?: string;
  permanentAddress?: string;
  emergencyContactName?: string;
  emergencyContactRelation?: string;
  emergencyContactPhone?: string;
  policeStation?: string;
  experience?: string;
  jobPost?: string;
  employmentStatus: EmployeeStatus;
  joiningDate?: string;
  exitDate?: string;
  essid?: string;
  photoPath?: string;
  dateOfBirth?: string;
  uan?: string;
  esiip?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface EmployeeListResponse {
  employees: Employee[];
  totalCount: number;
}

export interface EmployeeFormData {
  name: string;
  fatherName?: string;
  spouseName?: string;
  currentPlace?: string;
  currentPost?: string;
  currentAddress?: string;
  phoneNumbers: string[];
  permanentSameAsCurrent: number;
  permanentPlace?: string;
  permanentPost?: string;
  permanentAddress?: string;
  emergencyContactName?: string;
  emergencyContactRelation?: string;
  emergencyContactPhone?: string;
  policeStation?: string;
  experience?: string;
  jobPost?: string;
  employmentStatus: EmployeeStatus;
  joiningDate?: string;
  exitDate?: string;
  essid?: string;
  photoPath?: string;
  dateOfBirth?: string;
  uan?: string;
  esiip?: string;
}

export interface FilterOptions {
  query?: string;
  post?: string;
  jobPost?: string;
  exitDate?: string;
  joiningDate?: string;
  employmentStatus?: string;
}
