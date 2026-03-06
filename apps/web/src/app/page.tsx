import { Button, Card, CardContent, CardHeader, CardTitle, Badge } from "@restrosync/ui";
import { APP_NAME, APP_VERSION } from "@restrosync/shared";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-8 bg-bg">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle className="text-center">{APP_NAME}</CardTitle>
          <p className="text-center text-text-secondary text-sm">
            Next-Generation Restaurant Management Platform
          </p>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-center gap-2">
            <Badge variant="primary">v{APP_VERSION}</Badge>
            <Badge variant="success">Phase 0</Badge>
          </div>
          <div className="flex flex-col gap-3">
            <Button variant="primary" className="w-full">
              Get Started
            </Button>
            <Button variant="secondary" className="w-full">
              Documentation
            </Button>
          </div>
        </CardContent>
      </Card>
    </main>
  );
}
