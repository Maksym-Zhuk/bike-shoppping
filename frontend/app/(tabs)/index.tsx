import { SafeAreaView } from 'react-native-safe-area-context';
import HomePage from '../components/HomePage/HomePage';
export default function Index() {
  return (
    <>
      <SafeAreaView className="flex-1 bg-[#242C38]">
        <HomePage />
      </SafeAreaView>
    </>
  );
}
