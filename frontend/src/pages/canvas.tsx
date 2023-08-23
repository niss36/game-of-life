import { Inter } from "next/font/google";
import styles from "@/styles/Home.module.css";
import CanvasGameOfLife from "@/components/canvasLife";

const inter = Inter({ subsets: ["latin"] });

export default function WebGl() {
  return (
    <main className={`${styles.main} ${inter.className}`}>
      <CanvasGameOfLife />
    </main>
  );
}
