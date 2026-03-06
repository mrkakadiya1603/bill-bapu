import { Button, Card, CardContent, CardHeader, CardTitle, Badge } from "@restrosync/ui";
import { APP_NAME, APP_VERSION } from "@restrosync/shared";

function App() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-8 bg-bg">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle className="text-center">{APP_NAME}</CardTitle>
          <p className="text-center text-text-secondary text-sm">
            Desktop Application
          </p>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-center gap-2">
            <Badge variant="primary">v{APP_VERSION}</Badge>
            <Badge variant="success">Desktop</Badge>
          </div>
          <Button variant="primary" className="w-full">
            Launch Dashboard
          </Button>
        </CardContent>
      </Card>
    </main>
  );
}

export default App;
