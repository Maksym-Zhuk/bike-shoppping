import { View } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import HomePage from './components/HomePage/HomePage';
export default function Index() {
  return (
    <View className="flex-1 bg-[#242C38]">

      <SafeAreaView className="flex-1 px-4 bg-[#242C38]">

        <HomePage />

      </SafeAreaView>

    </View>
  );
}