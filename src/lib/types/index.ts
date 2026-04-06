export interface Service {
  id: string;
  name: string;
  url: string;
  icon: string | null;
  order: number;
  custom_css: string | null;
  custom_js: string | null;
  user_agent: string | null;
  zoom: number;
  notifications_enabled: boolean;
  is_active: boolean;
}