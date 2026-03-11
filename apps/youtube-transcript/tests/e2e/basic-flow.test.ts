import {test, expect} from '@playwright/test';

test.describe('YouTube Transcript Tool', () => {
  test.beforeEach(async ({page}) => {
    await page.goto('/');
  });

  test('should show title and language switcher', async ({page}) => {
    await expect(page.getByText('YouTube Transcript Tool')).toBeVisible();
    await expect(page.getByText('EN')).toBeVisible();
    await expect(page.getByText('中')).toBeVisible();
  });

  test('should validate invalid URL', async ({page}) => {
    await page.fill('input[type="url"]', 'https://example.com');
    await page.click('button[type="submit"]');
    await expect(page.getByText(/valid youtube url/i)).toBeVisible();
  });

  test('should switch language', async ({page}) => {
    await page.click('text=中');
    await expect(page.getByText('请输入 YouTube 视频链接')).toBeVisible();

    await page.click('text=EN');
    await expect(page.getByText(/enter youtube video url/i)).toBeVisible();
  });
});
