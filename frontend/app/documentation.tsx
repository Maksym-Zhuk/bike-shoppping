import { SafeAreaView } from 'react-native-safe-area-context';
import Navigation from './components/Navigation';
import DocumentationPage from './components/DocumentationPage/DocumentationPage';

export default function Documentation() {
    return (
        <>
            <SafeAreaView className="flex-1 bg-[#242C38]">
                <DocumentationPage />
            </SafeAreaView>
            <Navigation />
        </>
    );
}
